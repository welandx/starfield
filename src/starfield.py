sheng = {'s':'s','sh':'e','q':'q',
         'w':'w','r':'r','t':'t','y':'y','p':'p',
         's':'s','d':'d','f':'f','g':'g','h':'h',
         'j':'j','k':'k','l':'l','z':'z','x':'x',
         'c':'c','b':'b','n':'n','m':'m'}

yun = {'iu':'q',
       'ua':'q',
       'ei':'w',
       'un':'w',
       'e':'e',
       'eng':'r',
       'uan':'t',
       'iong':'y',
       'ong':'y',
       'ang':'p',
       'a':'s',
       'ia':'s',
       'ie':'d',
       'ou':'d',
       'an':'f',
       'ing':'g',
       'uai':'g',
       'ai':'h',
       'ue':'h',
       'er':'j',
       'u':'j',
       'i':'k',
       'o':'l',
       'uo':'l',
       'ao':'z',
       'iang':'x',
       'uang':'x',
       'iao':'c',
       'in':'b',
       'ui':'b',
       'en':'n',
       'ian':'m',
       'ü':'l',
       'üe':'h'}

for x in sheng:
    for y in yun:
        s=x+y
        v=sheng[x]+yun[y]
        match s:
            case 'ju','qu','xu','yu','ch','zh':
                pass
            case _:
                print(s,v)
